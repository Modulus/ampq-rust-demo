helm repo add bitnami https://charts.bitnami.com/bitnami

helm install quotes -f k8s/rabbitmq-helm-values.yaml bitnami/rabbitmq 

# To run 
cargo run -- --server="amqp://guest:guest@127.0.0.1:5672" --level=INFO

# Test quotes
cargo run --bin quotes

cargo run --bin producer -- --server="amqp://guest:guest@127.0.0.1:5672" --level=INFO