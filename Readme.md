test solo pod doc management

cargo run 

another terminal:

curl --request POST \
  --url 0.0.0.0:5100/miniopush \
  --header 'content-type: application/json' \
  --data '{
      "uid": "tress",
      "doc_name": "nameofmydoc",   
      "content_doc": "000000098765432232123456789"                 
  }'

  => push content doc on minio at the port 9000
  
