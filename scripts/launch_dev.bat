docker stop lexie_box
docker rm lexie lexie_box
docker image rm lexie
docker build -f ./envs/dev/Dockerfile -t lexie .
docker run -d --name lexie_box lexie
