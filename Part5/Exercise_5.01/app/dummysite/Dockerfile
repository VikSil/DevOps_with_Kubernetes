FROM nginx:1.19-alpine

WORKDIR /usr/share/nginx/html

COPY --chmod=777 curl.sh .

CMD ["sh", "/usr/share/nginx/html/curl.sh"]
