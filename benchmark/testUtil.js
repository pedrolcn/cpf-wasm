function testUtil(test_closure) {
  const TEST_CPF_COUNT = 10000000;
  const ONE_PERCENT = TEST_CPF_COUNT/100;

  for (let i=0; i< TEST_CPF_COUNT; i += 1) {
    if (i % ONE_PERCENT === 0) console.log(`${i/ONE_PERCENT} %`);
    const cpfString = i.toString().padStart(11, '0');
    test_closure(cpfString);
  }
}

module.exports = { testUtil };
