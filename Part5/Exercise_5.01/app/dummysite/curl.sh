if [ -n "$WEBSITE_URL" ]; then
  curl $WEBSITE_URL > index.html
  nginx -g 'daemon off;'
else
  printf "Environment wariable WEBSITE_URL missing\n"
  exit 1
fi
