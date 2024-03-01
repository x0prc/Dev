const bcrypt = require('bcrypt')
const hashPassword = async (pw) => {
    const salt = await bcrypt.genSalt(12);
    const hash = await bcrypt.hash(pw, salt);
    console.log(salt);
    console.log(hash);
}

const login = async(pw, hashedPw) => {
    const result = await bcrypt.compare(pw, hashedPw);
    if (result) {
        console.log('logged in ')
    } 
    else {
        console.log ('incorrect')
    }
}

hashPassword('banana');
login('banana', '$2b$12$fZ7zfVo2KFT1SXi6UFcxUO$2b$12$fZ7zfVo2KFT1SXi6UFcxUO1em2uw0L2oiH.bgMuDoNi/mRyfVx1rC' )