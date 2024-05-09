#ifndef USER_SERVICE_H
#define USER_SERVICE_H

#include "user.model.h"
#include <crow.h>
#include <crow/app.h>
#include <crow/http_response.h>
#include <crow/mustache.h>
#include <crow/utility.h>
#include <soci/mysql/soci-mysql.h>
#include <soci/soci.h>
#include <soci/sqlite3/soci-sqlite3.h>
#include <unordered_map>

class user_service {
public:
  user_service(env &env_) : env_(env_) { User::build(env_); }

  User create(const User &user) {
    soci::session sql(soci::mysql, env_.getDatabaseConnection());
    try {
      verifyConnection(sql);
      soci::statement st = sql.prepare
                           << "INSERT INTO users (username, password, role"
                           << (user.position ? ", position" : "")
                           << (user.firstName ? ", firstName" : "")
                           << (user.lastName ? ", lastName" : "")
                           << (user.bio ? ", bio" : "")
                           << (user.image ? ", image" : "")
                           << ") VALUES (:username, :password, :role"
                           << (user.position ? ", :position" : "")
                           << (user.firstName ? ", :firstName" : "")
                           << (user.lastName ? ", :lastName" : "")
                           << (user.bio ? ", :bio" : "")
                           << (user.image ? ", :image" : "") << ")";

      soci::indicator posInd = user.position ? soci::i_ok : soci::i_null;
      soci::indicator fNameInd = user.firstName ? soci::i_ok : soci::i_null;
      soci::indicator lNameInd = user.lastName ? soci::i_ok : soci::i_null;
      soci::indicator bioInd = user.bio ? soci::i_ok : soci::i_null;
      soci::indicator imgInd = user.image ? soci::i_ok : soci::i_null;

      st.exchange(soci::use(user.username, "username"));
      st.exchange(soci::use(user.password, "password"));
      st.exchange(soci::use(user.role, "role"));
      if (user.position)
        st.exchange(soci::use(user.position.value(), "position", posInd));
      if (user.firstName)
        st.exchange(soci::use(user.firstName.value(), "firstName", fNameInd));
      if (user.lastName)
        st.exchange(soci::use(user.lastName.value(), "lastName", lNameInd));
      if (user.bio)
        st.exchange(soci::use(user.bio.value(), "bio", bioInd));
      if (user.image)
        st.exchange(soci::use(user.image.value(), "image", imgInd));

      st.define_and_bind();
      st.execute(true);

      int lastId;
      sql << "SELECT LAST_INSERT_ID()", soci::into(lastId);
      sql.commit();
      return read(lastId);
    } catch (const std::exception &e) {
      std::cerr << "Error executing query: " << e.what() << std::endl;
      sql.rollback();
    }
  }

  User read(int id) {
    soci::session sql(soci::mysql, env_.getDatabaseConnection());
    try {
      verifyConnection(sql);
      User user;
      soci::indicator posInd, fNameInd, lNameInd, bioInd, imgInd;
      sql << "SELECT id, username, password, role, "
             "position, firstName, lastName, bio, image, "
             "UNIX_TIMESTAMP(createdAt), UNIX_TIMESTAMP(updatedAt) "
             "FROM users WHERE id = :id",
          soci::use(id), soci::into(user.id), soci::into(user.username),
          soci::into(user.password), soci::into(user.role),
          soci::into(user.position, posInd),
          soci::into(user.firstName, fNameInd),
          soci::into(user.lastName, lNameInd), soci::into(user.bio, bioInd),
          soci::into(user.image, imgInd), soci::into(user.createdAt),
          soci::into(user.updatedAt);

      if (posInd == soci::i_null)
        user.position.reset();
      if (fNameInd == soci::i_null)
        user.firstName.reset();
      if (lNameInd == soci::i_null)
        user.lastName.reset();
      if (bioInd == soci::i_null)
        user.bio.reset();
      if (imgInd == soci::i_null)
        user.image.reset();

      sql.close();
      return user;
    } catch (const std::exception &e) {
      std::cerr << "Error executing query: " << e.what() << std::endl;
    }
    sql.close();
    return User();
  }

  std::vector<User> readAll(const std::vector<int> &ids) {
    soci::session sql(soci::mysql, env_.getDatabaseConnection());
    std::vector<User> users;
    try {
      verifyConnection(sql);
      for (const auto &id : ids) {
        users.push_back(read(id));
      }
      sql.close();
      return users;
    } catch (const std::exception &e) {
      std::cerr << "Error executing query: " << e.what() << std::endl;
    }
    sql.close();
    return std::vector<User>();
  };

  std::vector<User> readAll() {
    soci::session sql(soci::mysql, env_.getDatabaseConnection());
    try {
      verifyConnection(sql);
      std::vector<User> users;
      soci::rowset<soci::row> rs =
          (sql.prepare
           << "SELECT id, username, password, role, position, firstName, "
              "lastName, bio, image, UNIX_TIMESTAMP(createdAt), "
              "UNIX_TIMESTAMP(updatedAt) FROM users");

      for (const auto &row : rs) {
        User user;
        user.username = row.get<std::string>("username", "");
        user.password = row.get<std::string>("password", "");
        user.role = row.get<std::string>("role", "");
        if (row.get_indicator("position") != soci::i_null)
          user.position = row.get<std::string>("position");
        if (row.get_indicator("firstName") != soci::i_null)
          user.firstName = row.get<std::string>("firstName");
        if (row.get_indicator("lastName") != soci::i_null)
          user.lastName = row.get<std::string>("lastName");
        if (row.get_indicator("bio") != soci::i_null)
          user.bio = row.get<std::string>("bio");
        if (row.get_indicator("image") != soci::i_null)
          user.image = row.get<std::string>("image");
        user.createdAt = row.get<std::time_t>("createdAt", std::time_t(0));
        user.updatedAt = row.get<std::time_t>("updatedAt", std::time_t(0));

        users.push_back(user);
      }
      sql.close();
      return users;
    } catch (const std::exception &e) {
      std::cerr << "Error executing query: " << e.what() << std::endl;
    }
    sql.close();
    return std::vector<User>();
  }

  User update(const User &user) {
    soci::session sql(soci::mysql, env_.getDatabaseConnection());
    try {
      verifyConnection(sql);
      soci::statement st =
          sql.prepare << "UPDATE users SET" << "username = :username,"
                      << "password = :password," << "role = :role,"
                      << (user.position ? "position = :position," : "")
                      << (user.firstName ? "firstName = :firstName," : "")
                      << (user.lastName ? "lastName = :lastName," : "")
                      << (user.bio ? "bio = :bio," : "")
                      << (user.image ? "image = :image," : "")
                      << "updatedAt = CURRENT_TIMESTAMP" << "WHERE id = :id";
      st.exchange(soci::use(user.username, "username"));
      st.exchange(soci::use(user.password, "password"));
      st.exchange(soci::use(user.role, "role"));
      if (user.position)
        st.exchange(soci::use(user.position.value(), "position"));
      if (user.firstName)
        st.exchange(soci::use(user.firstName.value(), "firstName"));
      if (user.lastName)
        st.exchange(soci::use(user.lastName.value(), "lastName"));
      if (user.bio)
        st.exchange(soci::use(user.bio.value(), "bio"));
      if (user.image)
        st.exchange(soci::use(user.image.value(), "image"));

      st.define_and_bind();
      st.execute(true);

      int lastId;
      sql << "SELECT LAST_INSERT_ID()", soci::into(lastId);

      sql.commit();
      sql.close();
      return read(lastId);
    } catch (const std::exception &e) {
      std::cerr << "Error executing query: " << e.what() << std::endl;
    }
    sql.close();
    return read(user.id);
  }

  User remove(int id) {
    User user = read(id);
    soci::session sql(soci::mysql, env_.getDatabaseConnection());
    try {
      verifyConnection(sql);
      sql << "DELETE FROM users WHERE id = :id", soci::use(id);
    } catch (const std::exception &e) {
      std::cerr << "Error executing query: " << e.what() << std::endl;
    }
    sql.close();
    return user;
  }

  std::vector<User> removeMultiple(const std::vector<int> &ids) {
    std::vector<User> users;
    users = readAll(ids);
    soci::session sql(soci::mysql, env_.getDatabaseConnection());
    try {
      verifyConnection(sql);
      for (const auto &id : ids) {
        sql << "DELETE FROM users WHERE id = :id", soci::use(ids);
      }
    } catch (const std::exception &e) {
      std::cerr << "Error executing query: " << e.what() << std::endl;
    }
    sql.close();
    return users;
  }

private:
  env &env_;
};

#endif // USER_SERVICE_H
