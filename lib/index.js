const express = require('express')
const bodyParser = require('body-parser');
const app = express()
const port = 3000

app.use(bodyParser.json());
app.get('/fib/:n', (req, res) => {
  const n = req.params["n"];

  const result = fib(n);

  res.json({ result });
})
app.post('/fib', (req, res) => {
  const n = req.body["n"];

  const result = fib(n);

  res.json({ result });
})

// Taken from https://medium.com/developers-writing/fibonacci-sequence-algorithm-in-javascript-b253dc7e320e
function fib(num){
  var a = 1, b = 0, temp;

  while (num >= 0){
    temp = a;
    a = a + b;
    b = temp;
    num--;
  }

  return b;
}

app.listen(port, () => console.log(`Example app listening on port ${port}!`))