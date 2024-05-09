#ifndef SERVICE_H
#define SERVICE_H

#include <crow.h>
#include <crow/app.h>
#include <crow/http_response.h>
#include <crow/mustache.h>
#include <crow/utility.h>
#include "utils.h"

class service {
public:
  void staticDir(const crow::request &req, crow::response &res,std::string fileName) {
    std::ifstream in("static/" + fileName, std::ifstream::in | std::ifstream::binary);
    if (in) {
      std::ostringstream contents;
      contents << in.rdbuf();
      in.close();
      res.body = contents.str();
      res.add_header("Content-Type", getContentType(fileName));
    } else {
      res.code = 404;
      res.body = "File not found";
    }
    res.end();
  }
private:
};

#endif // SERVICE_H
