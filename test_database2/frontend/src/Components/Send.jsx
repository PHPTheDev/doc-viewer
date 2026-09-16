import {useState, useRef } from 'react';
import './Send.css'
import axios from 'axios';

export default function Send() {

	const nameRef = useRef(null);
	const rfRef = useRef(null);
	const setorRef = useRef(null);

 	const button = async (e) => {
 		try {
 			const response = await axios.post(
 				"http://127.0.0.1:3000",{
 				"nome": `${nameRef.current.value}`,
 				"rf": `${rfRef.current.value}`,
 				"setor": `${setorRef.current.value}`
 				}
 			);
 		console.log(response.data);
 		}
 		catch(error) {
 			console.log(error);
 			console.log("explodiu")
 		} 
 		
 	}


	return (
		<>
		<div class="main">
			<h1>Pedido Papel</h1>
			<input type="text" placeholder="Nome" ref={nameRef} />
			<input type="text" placeholder="RF" ref={rfRef} />
			<input type="text" placeholder="Setor" ref={setorRef} />
			<input type="button" value="Enviar" onClick={button}/>
		</div>
		</>
		)
};
