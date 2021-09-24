import { io, Socket } from 'socket.io-client';
import { ClientEvents, ClientEventType, ServerEvents } from '@holochain/scaffolding-types';

export const socket: Socket = io();
