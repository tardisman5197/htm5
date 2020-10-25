# htm5
Another Chat App, here is the catch you can only send song lyrics.

## Prerequisits

* Docker
* Docker-Compose
* Make

## Building (with Make)

1. Move into the `server` directory

    ```
    cd ./server
    ```

2. Build the project

    ```
    make build
    ```

    or without Make

    ```
    sudo docker-compose build
    ```

## Running the server (with Make)

1. Move into the `server` directory

    ```
    cd ./server
    ```

2. Run the server

    ```
    make run
    ```

    This is now running in terminal.

    Alternatively you can run it in the background with:

    ```
    make start
    ```

3. Stop the server

    ```
    make stop
    ```

## Building (without Make)

1. Move into the `server` directory

    ```
    cd ./server
    ```

2. Build the project

    ```
    sudo docker-compose build
    ```

## Running the server (without Make)

1. Move into the `server` directory

    ```
    cd ./server
    ```

2. Run the server

    ```
    sudo docker-compose up
    ```

    This is now running in terminal.

    Alternatively you can run it in the background with:

    ```
    sudo docker-compse up -d
    ```

3. Stop the server

    ```
    sudo docker-compose down
    ```

## Usage of the server

The server will be running on `localhost:8080`

### Send Message

#### Method

`POST`

#### URL

`/message/add`

#### Body

```
{
	“message”: 	string,
	“sender”: 	string,
	“receiver”:	 string
}
```

### Get Messages

#### Method

`GET`

#### URL

`/messages/<sender>/<receiver>`

### Mark Message As Read

#### Method

`PATCH`

#### URL

`/message/<id>/read`

### Check Message Valid

#### Method

`POST`

### URL

`/message/valid`

### Body
```
{
	“message”: 	string
}
```
