#ifndef APP_SERVICE_H
#define APP_SERVICE_H 

#include <crow.h>
#include <crow/app.h>
#include <crow/http_response.h>
#include <crow/mustache.h>
#include <crow/utility.h>
#include <vector>
#include "../api/user/user.service.h"

class app_service {
public:

  app_service(env &env): userSvc(env) {};

  const crow::mustache::rendered_template home() {
    auto page = crow::mustache::load("base.html");
    auto child = crow::mustache::load("usersList.html");

    crow::mustache::context ctx;
    ctx["PUBLIC_URL"] = "/static";
    std::vector<User> users = userSvc.readAll();

    for (auto& user : users) {
      ctx["users"][user.id] = crow::mustache::context{
        {"id", user.id},
        {"username", user.username}
      };
    }
    ctx["content"] = child.render_string(ctx);

    return page.render(ctx);
  }

private:
  user_service userSvc;
};

#endif // APP_SERVICE_H  
