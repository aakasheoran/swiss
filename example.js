const fs = require('fs');

const contents = fs.readFileSync('data.json', 'utf8');
const data = JSON.parse(contents);

let inputs;
try {
  inputs = fs.readFileSync('inputs.json', 'utf8');
} catch (err) {
  fs.writeFileSync('inputs.json', '');
}

if (!(inputs === '')) fs.writeFileSync('inputs.json', '');

data.forEach(item => {
  const obj = JSON.parse(decodeURIComponent(item.input));
  const text = `{"_url":"${obj._url}", "organization_id":"${obj.organization_id}"}\n`;
  fs.appendFileSync('inputs.json', text.toString());
});

