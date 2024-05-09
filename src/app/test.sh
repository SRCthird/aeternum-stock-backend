#!/bin/bash

# Base URL of your Crow application
BASE_URL="http://localhost:18080/"

# Test home page connection
echo "Testing home page connection:"
curl "$BASE_URL"
echo -e "\n"
