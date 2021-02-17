<script lang="ts">
  import { Authenticator } from 'tauri-plugin-authenticator-api'

  const serverURL = 'http://localhost:3003/'
  const app = 'https://tauri.studio'

  const auth = new Authenticator()
  auth.init() // initialize usb
  
  let challenge = ''
  let keyHandle = ''
  let pubkey = ''
  let success = false

  async function register(){
    console.log('register!')
    const r = await fetch(serverURL+'challenge')
    const j = await r.json()
    challenge = j.challenge
    try {
      const registerData = await auth.register(challenge, app)
      console.log('=> registerData:', registerData)
      if(!r) return
      const r2 = await fetch(serverURL+'register', {
        method:'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify({challenge, registerData})
      })
      const j2 = await r2.json()
      console.log('=> /register result:', j2)
      keyHandle = j2.keyHandle
      pubkey = j2.pubkey
    } catch(e) {
      console.log("ERROR registering:",e)
    }
  }

  async function sign(){
    try {
      const signData = await auth.sign(challenge, app, keyHandle)
      console.log('=> signData:', signData)
      const r = await fetch(serverURL+'verify', {
        method:'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify({challenge, signData, pubkey})
      })
      const j = await r.json()
      console.log('=> sign result:', j)
      if(j.success) success=true
    } catch(e) {
      console.log("ERROR signing:",e)
    }
  }

  $: hasKeyHandle = keyHandle?true:false
</script>

<main>
	<h1>Tauri Authenticator</h1>
  <br />
  <section>

    <button on:click={register} disabled={hasKeyHandle}>REGISTER</button>

    <button on:click={sign} disabled={!hasKeyHandle}>SIGN</button>

    {#if success}
      <span>SUCCESS!</span>
    {/if}

  </section>
  <br />
  <section>
    <div>KEY HANDLE:</div>
    <pre>{keyHandle}</pre>
    <div>PUBLIC KEY:</div>
    <pre>{pubkey}</pre>
  </section>
</main>

<style>
  button{
    width:100px;
    height:42px;
    border-radius: 10px;
    outline:none;
    border:1px solid grey;
    cursor: pointer;
  }
  span{
    font-weight: bold;
    margin-left:20px;
    font-size:24px;
  }
</style>