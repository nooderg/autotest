FROM rustlang/rust:nightly

WORKDIR /app
COPY . .

EXPOSE 8000

CMD [ "cargo", "run" ] 
