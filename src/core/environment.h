#ifndef ENVIRONMENT_H
#define ENVIRONMENT_H

#include <map>
#include <string>
#include <crow.h>
#include <crow/app.h>
#include <crow/http_response.h>
#include <unistd.h>

class env {
public:
  env(std::string envFile);
  std::string getEnv(const std::string &key);
  int getPort();
  bool getHttps();
  std::string getHost();
  std::string getCert();
  std::string getKey();
  std::string getPem();
  std::string getHostname();
  std::string getDatabaseConnection();

private:
  std::string databaseConnection;
  std::string fileName;
  std::map<std::string, std::string> envVariables;
  std::map<std::string, std::string> loadEnvVariables();
  std::string HOST = "localhost";
  int PORT = 8080;
  std::string SSL_CERT;
  std::string SSL_KEY;
  std::string SSL_PEM;
  bool HTTPS = false;
};

#endif // ENVIRONMENT_H
