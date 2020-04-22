const cpfCheck = require('cpf-check');
const { testUtil } = require('./testUtil');

testUtil(cpfCheck.validate);
