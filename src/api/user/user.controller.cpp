#include "user.controller.h"
#include "user.model.h"
#include "user.service.h"

user_controller::user_controller(RouteManager &r, env &env_): r(r), env_(env_) {
  user_controller::setRoot("/user");
  user_service userService(env_);

  addRoute("GET", "hello")
  ([](const crow::request &req) {
    return crow::response(200, "hello");
  });

  addRoute("POST", "/")
  ([&userService](const crow::request &req) {
    User user;
    try {
      crow::json::rvalue json = crow::json::load(req.body);
      User data(json);
      user = userService.create(data);
    } catch (const std::exception &e) {
      return crow::response(500, e.what());
    }
    return crow::response(200, user.toJson().dump());
  });

  addRoute("POST", "/bulk")
  ([this, &userService](const crow::request &req) {
    std::vector<User> newUsers;
    try {
      crow::json::rvalue json = crow::json::load(req.body);
      for (const auto &user : json) {
        if (user.has("username")) {
          newUsers.push_back(userService.create(User(user)));
        } 
      }
    } catch (const std::exception &e) {
      return crow::response(500, e.what());
    }
    std::vector<crow::json::wvalue> jsonResponse;
    for (const auto &user : newUsers) {
      jsonResponse.push_back(user.toJson());
    }

    return crow::response(200, crow::json::wvalue(jsonResponse).dump());
  });

  addRoute("GET", "/<int>")
  ([&userService](int id) {
    try {
      User user = userService.read(id);
      if (user.id == 0) {
        return crow::response(404, "User not found");
      }
      return crow::response(200, user.toJson().dump());
    } catch (const std::exception &e) {
      return crow::response(500, e.what());
    }
  });

  addRoute("GET", "/")
  ([&userService](const crow::request &req) {
    try {
      std::vector<int> ids;
      const char* idParam = req.url_params.get("id");
      if (idParam != nullptr) {
        std::stringstream ss(idParam);
        std::string id;
        while (std::getline(ss, id, ',')) {
          ids.push_back(std::stoi(id));
        }
      }

      std::vector<User> users;
      if (!ids.empty()) {
        users = userService.readAll(ids);
      } else {
        users = userService.readAll();
      }

      std::vector<crow::json::wvalue> jsonResponse;
      for (const auto &user : users) {
        jsonResponse.push_back(user.toJson());
      }

      return crow::response(200, crow::json::wvalue(jsonResponse).dump());
    } catch (const std::exception &e) {
      return crow::response(404, e.what());
    }
  });

  addRoute("PATCH", "/")
  ([&userService](const crow::request &req) {
    User newUser;
    try {
      crow::json::rvalue json = crow::json::load(req.body);
      newUser = userService.update(User(json));
    } catch (const std::exception &e) {
      return crow::response(500, e.what());
    }
    return crow::response(200, newUser.toJson().dump());
  });

  addRoute("DELETE", "/<int>")
  ([&userService](int id) {
    User user;
    try {
      user = userService.remove(id);
      if (user.id == 0) {
        return crow::response(404, "User not found");
      }
    } catch (const std::exception &e) {
      return crow::response(500, e.what());
    }
    return crow::response(200, user.toJson().dump());
  });

  addRoute("DELETE", "/")
  ([&userService](const crow::request &req) {
    std::vector<User> users;
    try {
      std::vector<int> ids;
      const char* idParam = req.url_params.get("id");
      if (idParam != nullptr) {
        std::stringstream ss(idParam);
        std::string id;
        while (std::getline(ss, id, ',')) {
          ids.push_back(std::stoi(id));
        }
      }
      if (!ids.empty()) {
        users = userService.removeMultiple(ids);
      } else {
        throw std::runtime_error("No ids provided");
      }
    } catch (const std::runtime_error &e) {
      return crow::response(400, e.what());
    } catch (const std::exception &e) {
      return crow::response(500, e.what());
    }
    std::vector<crow::json::wvalue> jsonResponse;
    for (const auto &user : users) {
      jsonResponse.push_back(user.toJson());
    }

    return crow::response(200, crow::json::wvalue(jsonResponse).dump());
  });
}
