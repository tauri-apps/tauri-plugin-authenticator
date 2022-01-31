<script lang="ts">
	import { Authenticator } from 'tauri-plugin-authenticator-api'
  
	const app = 'https://tauri.studio'
  
	const auth = new Authenticator()
	auth.init() // initialize usb
	
	let challenge = ''
	let keyHandle = ''
	let deviceName = ''
	let pubkey = ''
	let clientData = ''
	let success = false
  
	function genChallenge(): Promise<string> {
	  return new Promise((resolve,reject)=>{
		var arr = new Uint8Array(32);
		window.crypto.getRandomValues(arr)
		const b64 = btoa(String.fromCharCode.apply(null, arr))
		// web-safe base64
		resolve(b64.replace(/\+/g, '-').replace(/\//g, '_'));
	  })
	}
  
	async function register(){
	  console.log('register!')
	  const chall = await genChallenge()
	  console.log("challenge!", chall)
	  challenge = chall
	  try {
		const json = await auth.register(challenge, app)
		console.log('=> registerData:', json)
		const registerResult = JSON.parse(json)
		if(!registerResult.pubkey || !registerResult.keyHandle || !registerResult.clientData || !registerResult.registerData) {
		  return console.log("MISSING FIELDS IN REGISTER RESULT")
		}
		clientData = registerResult.clientData
		const r2 = await auth.verifyRegistration(challenge, app, registerResult.registerData, registerResult.clientData)
		console.log("R2", r2)
		const j2 = JSON.parse(r2)
		if(!j2.pubkey || !j2.keyHandle || !j2.deviceName) {
		  return console.log("MISSING FIELDS IN REGISTER VERIFICATION RESULT")
		}
		keyHandle = j2.keyHandle
		pubkey = j2.pubkey
		deviceName = j2.deviceName
	  } catch(e) {
		console.log("ERROR registering:",e)
	  }
	}
  
	async function sign(){
	  try {
		const json = await auth.sign(challenge, app, keyHandle)
		console.log('=> signData:', json)
		const signData = JSON.parse(json)
		if(!signData.signData || !signData.keyHandle) {
		  return console.log("MISSING FIELDS IN SIGN RESULT")
		}
  
		const counter = await auth.verifySignature(challenge, app, signData.signData, clientData, keyHandle, pubkey)
		console.log(counter)
		if(counter && counter>0) {
		  success=true
		}
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
	  <div>DEVICE NAME:</div>
	  <pre>{deviceName}</pre>
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