# AI10cle-API Documentation.


# Schema

## Users

1. ## Input User

> These are the properties of the user struct that will be inserted into the database.

| Field      | Type      | Table Notes               |
|------------|-----------|---------------------------|
| id         | serial    | Not Nullable, Primary Key |
| user_id    | i32       | Not Nullable, Integer     |
| title      | string    | Text, Not Null            |
| first_name | string    | Text, Not Null            |
| last_name  | string    | Text, Not Null            |
| password   | string    | Text, Not Null            |
| email      | string    | Text, Not Null            |
| created_at | timestamp | Text, Not Null            |

2. ## Auth User

> These are the properties of the user struct that will be used to authenticate against the database and initiate a session.

| Field      | Type   | Table Notes               |
|------------|--------|---------------------------|
| password   | string | Text, Not Null            |
| email      | string | Text, Not Null            |

## Articles

1. ## Input Article
   
> These are the properties of the article struct that will be inserted into the database

| Field      | Type      | Table Notes               |
|------------|-----------|---------------------------|
| id         | i32       | Integer, Not Null         |
| user_id    | i32       | Integer, Not Null         |
| title      | string    | Text, Not Null            |
| link       | string    | Text, Not Null            |


## Base Url: https://ai10cle-api.herokuapp.com/api/

## Install

    cargo install

## Run the app

    cargo run

## Run the tests

    cargo test

# REST API

The REST API to the example app is described below.

## Get list of Users

### Request

`GET /users/`

    curl -i -H 'Accept: application/json' https://ai10cle-api.herokuapp.com/api/users

### Response

    HTTP/1.1 200 OK
    Date: Thu, 24 Feb 2011 12:36:30 GMT
    Status: 200 OK
    Connection: close
    Content-Type: application/json
    Content-Length: 2

## Create a new User

### Request

`POST /users/`

    curl -i -H 'Accept: application/json' -d 'name=Foo&status=new' https://ai10cle-api.herokuapp.com/api/users

### Response

    HTTP/1.1 201 Created
    Date: Thu, 24 Feb 2011 12:36:30 GMT
    Status: 201 Created
    Connection: close
    Content-Type: application/json
    Location: /thing/1
    Content-Length: 36

    {"id":1,"name":"Foo","status":"new"}

## Get a specific User

### Request

`GET /users/id`


### Response

    HTTP/1.1 200 OK
    Date: Thu, 24 Feb 2011 12:36:30 GMT
    Status: 200 OK
    Connection: close
    Content-Type: application/json
    Content-Length: 36

    {"id":1,"name":"Foo","status":"new"}

## Get an Article by id.

### Request

`GET /articles/id`

    curl -i -H 'Accept: application/json' https://ai10cle-api.herokuapp.com/api/users/id

### Response

    HTTP/1.1 404 Not Found
    Date: Thu, 24 Feb 2011 12:36:30 GMT
    Status: 404 Not Found
    Connection: close
    Content-Type: application/json
    Content-Length: 35

    {"status":404,"reason":"Not found"}

## Create another new Thing

### Request

`POST /article/`

    curl -i -H 'Accept: application/json' -d 'name=Bar&junk=rubbish' https://ai10cle-api.herokuapp.com/api/articles

### Response

    HTTP/1.1 201 Created
    Date: Thu, 24 Feb 2011 12:36:31 GMT
    Status: 201 Created
    Connection: close
    Content-Type: application/json
    Location: /thing/2
    Content-Length: 35

    {"id":2,"name":"Bar","status":null}

## Get list of Articles

### Request

`GET /article/`

    curl -i -H 'Accept: application/json' https://ai10cle-api.herokuapp.com/api/articles

### Response

    HTTP/1.1 200 OK
    Date: Thu, 24 Feb 2011 12:36:31 GMT
    Status: 200 OK
    Connection: close
    Content-Type: application/json
    Content-Length: 74

    [{"id":1,"name":"Foo","status":"new"},{"id":2,"name":"Bar","status":null}]



## Get Article

### Request

`GET /article/id`

    curl -i -H 'Accept: application/json' https://ai10cle-api.herokuapp.com/api/articles/id

### Response

    HTTP/1.1 200 OK
    Date: Thu, 24 Feb 2011 12:36:31 GMT
    Status: 200 OK
    Connection: close
    Content-Type: application/json
    Content-Length: 40

    {"id":1,"name":"Foo","status":"changed"}

