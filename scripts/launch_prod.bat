docker stop lexie_box
docker rm lexie_box
docker image rm lexie
docker build -f ./envs/prod/Dockerfile -t lexie .
docker run -d --name lexie_box lexie