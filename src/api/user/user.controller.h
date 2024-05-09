#ifndef USER_CONTROLLER_H
#define USER_CONTROLLER_H 

#include "../../core/environment.h"
#include "../../core/routeManager.h"
#include <string>

class user_controller {
public:
  user_controller(RouteManager &r, env &env_);
  void setRoot(std::string root) { this->root += root; }
  std::string route(const std::string append) { 
    if (this->root.back() == '/' && append[0] == '/') {
      return this->root + append.substr(1); 
    } else if (this->root.back() != '/' && append[0] != '/') {
      return this->root + "/" + append; 
    }
    return this->root + append; 
  }
  std::string route() { return this->root; }

  crow::DynamicRule& addRoute(const std::string& method, const std::string& path) {
    return r.addRoute(method, this->route(path));
  }

  crow::DynamicRule& addRoute(crow::HTTPMethod& method, const std::string& path) {
    return r.addRoute(method, this->route(path));
  }

  crow::DynamicRule& addRoute(const std::string& path) {
    return r.addRoute(this->route(path));
  }

private:
  RouteManager &r;
  env &env_;
  std::string root = "/api";
};

#endif // USER_CONTROLLER_H 
