
# Autotest API

This is a REST API which was written in Rust, following a Domain Driven Design architecture. It includes CRUD routes for users.


## API Reference

#### Get user

```http
  GET /users/${id}
```

| Parameter | Type     | Description                |
| :-------- | :------- | :------------------------- |
| `id` | `Uuid` | **Required**. The id of the user |

#### Create user

```http
  POST /users
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `first_name`      | `string` | **Required**. First name of the user |
| `last_name`      | `string` | **Required**. Last name of the user |
| `email`      | `string` | **Required**. Email address of the user |
| `password`      | `string` | **Required**. Password of the user |

#### Delete user

```http
  DELETE /users/${id}
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `id`      | `Uuid` | **Required**. The id of the user |

#### Update user

```http
  PUT /users/${id}
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `id`      | `Uuid` | **Required**. The id of the user |
| `first_name`      | `string` | First name of the user |
| `last_name`      | `string` | Last name of the user |
| `email`      | `string` | Email address of the user |
| `password`      | `string` | Password of the user |