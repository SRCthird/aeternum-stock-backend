#ifndef UTILS_H
#define UTILS_H

#include <chrono>
#include <soci/session.h>
#include <string>

inline std::string getDateNow() {
  auto now = std::chrono::system_clock::now();
  std::time_t now_time = std::chrono::system_clock::to_time_t(now);
  std::tm now_tm = *std::localtime(&now_time);
  char buffer[80];
  std::strftime(buffer, sizeof(buffer), "%Y-%m-%d %H:%M:%S", &now_tm);
  return std::string(buffer);
}

inline std::string getContentType(const std::string &fileName) {
  if (fileName.rfind(".png") == fileName.length() - 4) {
    return "image/png";
  } else if (fileName.rfind(".ico") == fileName.length() - 4) {
    return "image/vnd.microsoft.icon";
  } else if (fileName.rfind(".jpg") == fileName.length() - 4) {
    return "image/jpeg";
  } else if (fileName.rfind(".jpeg") == fileName.length() - 5) {
    return "image/jpeg";
  } else if (fileName.rfind(".js") == fileName.length() - 3) {
    return "text/javascript";
  } else if (fileName.rfind(".css") == fileName.length() - 4) {
    return "text/css";
  } else if (fileName.rfind(".json") == fileName.length() - 5) {
    return "application/json";
  } else {
    return "application/octet-stream";
  }
}

inline void verifyConnection(soci::session &sql) {
  if (!sql.is_connected()) {
    throw std::runtime_error("Database is not connected.");
  }
}

#endif // UTILS_H
