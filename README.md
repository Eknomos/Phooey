This project is under development. To run the web version on your local machine follow these initialization steps

1. Install Rust if you haven't already done so
2. Install Docker
3. Run Docker 
4. Run the scripts/init_postgres.sh file to create a development Postgres database in docker
5. Run the scripts/init_redis.sh file to create development Redis database
6. You will need to uncomment lines #8 and #18 in main.rs to initialize the tables in the database
7. Run main.rs
8. Navigate to localhost:8000 to access the website
