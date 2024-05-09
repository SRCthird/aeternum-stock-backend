#include "app.controller.h"
#include "app.service.h"


app_controller::app_controller(RouteManager &r, env &env_): r(r), env_(env_) {
  app_controller::setRoot("/");
  app_service appService(env_);

  addRoute(app_controller::route())
  ([&appService](const crow::request &req) {
    return crow::response(200, appService.home().dump());
  });
}
