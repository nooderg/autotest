# Autotest

## 💾 Automated && managed integration test suite from an .openapi file
Autotest's goal is to bring integration testing to it's most automated and easiest state. 
Simply annotate your code with Swagger to generate an `.openfile` file, send it to us, and you are done!

## 🚀 So, where is the product?
Autotest is still under development. We are still making features.
- [x] Accounts
- [x] Tavern file generation
- [ ] CLI
- [ ] Managed && automated testing instances
- [ ] Statistics & API benchmark

## 💸 And for what price?
We did not decide ourselves on pricing yet. We'd like to have a free tier so you can try the product and decide if you want to adopt us!

## 🏃 Okay, how do I run it?
You will need Rust (cargo), and diesel CLI.
When this is installed, run the migration on the database:

Start the database with:

```bash
docker-compose -f docker-compose.staging.yaml up autotest_db
```

Run the migration
```bash
diesel database setup --database-url postgres://developer:developer@localhost:5432/autotest
```

Now that the setup is done, close your container and run `make staging`

## 👉👈 Awesome! Can I have the API descrition?
Yup, here it is: https://www.getpostman.com/collections/34af2658a9bd81eb244c