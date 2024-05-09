#include "environment.h"


env::env(std::string envFile) : fileName(envFile), envVariables(loadEnvVariables()) {
  try {
      env::HOST = env::getEnv("HOST");
  } catch (const std::invalid_argument &e) {
    std::cerr << "HOST Configuration Error: " << e.what() << std::endl;
  }
  try {
    auto httpsValue = env::getEnv("HTTPS");
    env::HTTPS = httpsValue == "1" || httpsValue == "true";
  } catch (const std::invalid_argument &e) {
    std::cerr << "HTTPS Configuration Error: " << e.what() << std::endl;
  }
  if (env::HTTPS) {
    try {
      env::SSL_CERT = env::getEnv("SSL_CERT");
      env::SSL_KEY = env::getEnv("SSL_KEY");
    } catch (const std::invalid_argument &certKeyException) {
      try {
        env::SSL_PEM = env::getEnv("SSL_PEM");
      } catch (const std::invalid_argument &pemException) {
        std::cerr << "SSL Configuration Error: " << pemException.what() << std::endl;
      }
    }
  }
  try {
    env::PORT = std::stoi(env::getEnv("PORT"));
  } catch (const std::invalid_argument &e) {
    std::cerr << "PORT Configuration Error: " << e.what() << std::endl;
    env::PORT = env::HTTPS ? 443 : 80;
  } catch (const std::out_of_range &e) {
    std::cerr << "PORT Configuration Error: Specified value is out of range." << std::endl;
    env::PORT = env::HTTPS ? 443 : 80;
  }
  try {
    env::databaseConnection = env::getEnv("DATABASE_CONNECTION");
  } catch (const std::invalid_argument &e) {
    std::cerr << "Database Configuration Error: " << e.what() << std::endl;
  }
}

int env::getPort() {
    return env::PORT;
};
bool env::getHttps(){
    return env::HTTPS;
};
std::string env::getHost(){
    return env::HOST;
};
std::string env::getCert(){
    return env::SSL_CERT;
};
std::string env::getKey(){
    return env::SSL_KEY;
};
std::string env::getPem(){
    return env::SSL_PEM;
};
std::string env::getDatabaseConnection(){
    return env::databaseConnection;
};

std::string env::getEnv(const std::string &key) {
  auto it = envVariables.find(key);
  if (it == envVariables.end() || it->second.empty()) {
    throw std::invalid_argument("Could not find environment variable: " + key);
  }
  return it->second;
}

std::map<std::string, std::string> env::loadEnvVariables() {
  std::map<std::string, std::string> env;
  std::ifstream file(env::fileName);
  std::string line;

  while (std::getline(file, line)) {
    std::istringstream is_line(line);
    std::string key;
    if (std::getline(is_line, key, '=')) {
      std::string value;
      if (key[0] == '#')
        continue;
      if (key[0] == '\n')
        continue;
      if (std::getline(is_line, value)) {
        // Trim whitespace from key
        key.erase(key.find_last_not_of(" \n\r\t") + 1);

        // Trim whitespace and quotation marks from value
        value.erase(0, value.find_first_not_of(" \n\r\t\""));
        value.erase(value.find_last_not_of(" \n\r\t\"") + 1);

        env[key] = value;
      }
    }
  }

  return env;
}

std::string env::getHostname() {
  if (env::HOST != "localhost") return env::HOST;
  char hostname[1024];
  if (gethostname(hostname, sizeof(hostname)) != 0) {
    std::cerr << "Failed to get hostname" << std::endl;
    return "";
  }

  struct addrinfo hints, *info;
  memset(&hints, 0, sizeof(hints));
  hints.ai_family = AF_INET;
  hints.ai_socktype = SOCK_STREAM;
  hints.ai_flags = AI_CANONNAME;

  if (getaddrinfo(hostname, nullptr, &hints, &info) != 0) {
    std::cerr << "Failed to get addrinfo" << std::endl;
    return "";
  }

  std::string fqdn(info->ai_canonname);
  freeaddrinfo(info);
  return fqdn;
}
