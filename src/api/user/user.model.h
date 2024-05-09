#pragma once

#include "../../core/environment.h"
#include "../../core/utils.h"
#include <boost/optional.hpp>
#include <crow.h>
#include <crow/json.h>
#include <ctime>
#include <optional>
#include <soci/mysql/soci-mysql.h>
#include <soci/session.h>
#include <string>
#include <unordered_map>

class User {
public:
  User() = default;
  User(const crow::json::rvalue &json) {
    if (json.has("id")) {
      id = json["id"].i();
    }
    username = json["username"].s();
    password = json["password"].s();
    role = json["role"].s();
    if (json.has("position")) {
      position = json["position"].s();
    }
    if (json.has("firstName")) {
      firstName = json["firstName"].s();
    }
    if (json.has("lastName")) {
      lastName = json["lastName"].s();
    }
    if (json.has("bio")) {
      bio = json["bio"].s();
    }
    if (json.has("image")) {
      image = json["image"].s();
    }
  }
  ~User() = default;

  // Data Structure
  int id = 0;
  std::string username;
  std::string password;
  std::string role;
  boost::optional<std::string> position;
  boost::optional<std::string> firstName;
  boost::optional<std::string> lastName;
  boost::optional<std::string> bio;
  boost::optional<std::string> image;
  std::time_t createdAt;
  std::time_t updatedAt;

  static void build(env &env_) {
    soci::session sql(soci::mysql, env_.getDatabaseConnection());
    try {
      verifyConnection(sql);
      sql << "CREATE TABLE IF NOT EXISTS \"users\" ("
          << "  \"id\" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,"
          << "  \"username\" TEXT NOT NULL," << "  \"password\" TEXT NOT NULL,"
          << "  \"role\" TEXT NOT NULL DEFAULT 'Operator',"
          << "  \"position\" TEXT," << "  \"firstName\" TEXT,"
          << "  \"lastName\" TEXT," << "  \"bio\" TEXT," << "  \"image\" TEXT,"
          << "  \"createdAt\" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,"
          << "  \"updatedAt\" DATETIME NOT NULL" << ");";
    } catch (const std::exception &e) {
      std::cerr << "Error creating table: " << e.what() << std::endl;
    }
    sql.close();
  }

  crow::json::wvalue toJson() const {
    crow::json::wvalue response;
    response["id"] = id;
    response["username"] = username;
    response["role"] = role;

    if (position)
      response["position"] = *position;
    if (firstName)
      response["firstName"] = *firstName;
    if (lastName)
      response["lastName"] = *lastName;
    if (bio)
      response["bio"] = *bio;
    if (image)
      response["image"] = *image;

    char buf[80];
    std::strftime(buf, sizeof(buf), "%Y-%m-%d %H:%M:%S",
                  std::localtime(&createdAt));
    response["createdAt"] = std::string(buf);
    std::strftime(buf, sizeof(buf), "%Y-%m-%d %H:%M:%S",
                  std::localtime(&updatedAt));
    response["updatedAt"] = std::string(buf);

    return response;
  }

  operator crow::json::wvalue() const { return toJson(); }

private:
};
