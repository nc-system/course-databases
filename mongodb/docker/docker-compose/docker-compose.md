
version: "3.8"

services:

  nc-dev-front:
      container_name: nc-dev-full-stack
      restart: always
      build: .
      environment: 
        - MARIADB_URI=mariadb://mariadb:3306/nc-db-dev-mariadb-test-01
        - PORT= 3306
        - NODE_ENV= development

        - MARIADB_URI=mongodb://mongodb:27018/nc-db-dev-mariadb-test-01
        - PORT= 27018
        - NODE_ENV= development

      depends_on:
        - mariadb
      #volumes:
      #  - .vscode:/home/node
      ports: ["22:22"]
      tty: true
     
      
  mariadb:
      container_name: nc-db-dev-mariadb-10.7.3
      image: mariadb:10.7.3-focal
      environment:
        MARIADB_DATABASE: 'nc-db-dev-mariadb-test-01'
        MARIADB_USER: 'newsoftcomputer'
        MARIADB_PASSWORD: 'newsoftcomputer'
        MARIADB_ROOT_PASSWORD: 'newsoftcomputer'
      ports:
        # <Port exposed> : < MySQL Port running inside container>
        - '3306:3306'
        - '3305:3305'
        - '3307:3307'
      volumes:
        - "./nc-db-mariadb:/databases/mariadb"
        #- my-db:/var/lib/mariadb


  mongodb:
    container_name: nc-db-dev-mongo:5.0.8-focal
    image: mongo:5.0.8-focal
    environment:
      MONGO_DATABASE: ''
      MONGO_USER: ''
      MONGO_PASSWORD: ''
      MONGO_ROOT_PASSWORD: ''
    ports:
      - '27017:27017'
      - '27018:27018'
      - '27019:27019'
    volumes:
      - "./nc-db-mongodb:/databases/mongodb"