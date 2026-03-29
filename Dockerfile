FROM rust:latest-slim
WORKDIR /app
COPY . .
COPY .github/credentials.yml config/db.yml
RUN make all
EXPOSE 5000
CMD ["./server"]
