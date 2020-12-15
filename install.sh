#!/bin/bash

get_os () {
  if [[ "$OSTYPE" == "linux-gnu" ]]; then
    echo "linux"
  elif [[ "$OSTYPE" == "freebsd"* ]]; then
    echo "linux"
  elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo "osx"
  elif [[ "$OSTYPE" == "cygwin" ]]; then
    echo "windows"
  elif [[ "$OSTYPE" == "msys" ]]; then
    echo "windows"
  elif [[ "$OSTYPE" == "win32" ]]; then
    echo "windows"
  else
    return -1
  fi
}

cd /tmp
os=`get_os`
echo "Downloading latest ${os} release..."
curl -sLo comodoro.tar.gz "https://github.com/soywod/comodoro/releases/latest/download/comodoro-${os}.tar.gz"
echo "Installing binaries..."
tar -xzf comodoro.tar.gz
rm comodoro.tar.gz
chmod u+x comodoro*
sudo mv comodoro* /usr/local/bin/

echo "Comodoro installed!"
