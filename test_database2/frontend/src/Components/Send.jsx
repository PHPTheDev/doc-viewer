import {useState, useRef } from 'react';
import './Send.css'
import Axios from 'axios';
import api from '../lib/axios';




export default function Send() {

	const nameRef = useRef(null);
	const rfRef = useRef(null);
	const setorRef = useRef(null);

 	const button = (e) => {
 		console.log(nameRef.current.value);
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
