#include "core/controller.h"
#include "core/routeManager.h"
#include "api/user/user.controller.h"
#include "app/app.controller.h"

int main() {
  crow::SimpleApp app;
  env env(".env");
  RouteManager routeManager(app);
  
  // Import Controllers
  controller ctrl(app, env, routeManager);
  user_controller user_ctrl(routeManager, env);
  app_controller app_ctrl(routeManager, env);
  // End Import Controllers
  
  routeManager.printRoutes();
  ctrl.run();
}
