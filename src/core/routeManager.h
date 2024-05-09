#ifndef ROUTE_MANAGER_H
#define ROUTE_MANAGER_H

#include <cctype>
#include <crow.h>
#include <crow/app.h>
#include <crow/common.h>
#include <crow/routing.h>
#include <crow/utility.h>
#include <vector>
#include <string>


class RouteManager {
public:
  RouteManager(crow::SimpleApp& app) : app_(app) {}

  std::unordered_map<std::string, crow::HTTPMethod> httpMethodMap = {
    {"GET", crow::HTTPMethod::GET},
    {"POST", crow::HTTPMethod::POST},
    {"PUT", crow::HTTPMethod::PUT},
    {"DELETE", crow::HTTPMethod::DELETE},
    {"PATCH", crow::HTTPMethod::PATCH},
    {"HEAD", crow::HTTPMethod::HEAD},
    {"OPTIONS", crow::HTTPMethod::OPTIONS}
  };

  crow::HTTPMethod methodFromString(const std::string& method) {
    std::string upperMethod;
    for (char c : method) {
        upperMethod += std::toupper(c);
    }
    auto it = httpMethodMap.find(upperMethod);
    if (it != httpMethodMap.end()) {
        return it->second;
    }
    return crow::HTTPMethod::GET;
  }

  crow::DynamicRule& addRoute(const std::string& method, const std::string& path) {
    crow::HTTPMethod crowMethod = methodFromString(method);
    routes_.push_back(method + " " + path);
    return app_.route_dynamic(std::string(path)).methods(crowMethod);
  }

  crow::DynamicRule& addRoute(crow::HTTPMethod& method, const std::string& path) {
    routes_.push_back(crow::method_name(method) + " " + path);
    return app_.route_dynamic(std::string(path)).methods(method);
  }

  crow::DynamicRule& addRoute(const std::string& path) {
    routes_.push_back("GET " + path);
    return app_.route_dynamic(std::string(path));
  }

  void printRoutes() const {
    for (const auto& route : routes_) {
      std::cout << route << std::endl;
    }
  }

private:
  crow::SimpleApp& app_;
  std::vector<std::string> routes_;
};

#endif // ROUTE_MANAGER_H

