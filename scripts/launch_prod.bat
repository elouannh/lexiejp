docker stop lexie_prod_box
docker rm lexie_prod_box
docker image rm lexie_prod
rmdir /s /q .\target\
docker build -f ./envs/prod/Dockerfile -t lexie_prod .
docker run -d --name lexie_prod_box lexie_prod
