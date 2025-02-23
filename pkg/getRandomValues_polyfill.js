globalThis.crypto = {
  getRandomValues(abv) {
    let counter = abv.length;
    while (counter--) {
      abv[counter] = Math.floor(Math.random() * 256);
    }
    return abv;
  },
};
