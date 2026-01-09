FROM nginx:1.20

COPY docker/nginx-default.conf.template /etc/nginx/conf.d/default.conf.template

COPY docker/docker-entrypoint.sh /
ENTRYPOINT ["/docker-entrypoint.sh"]
CMD ["nginx", "-g", "daemon off;"]

COPY html /usr/share/nginx/html
