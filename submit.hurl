# hurl --error-format long --variable serviceUrl="https://advent-of-spin-challenge-two-lph7y6ht.fermyon.app" submit.hurl
POST https://advent-of-spin2023.fermyon.app/2
{
  "serviceUrl": "{{serviceUrl}}"
}

HTTP 200