/* tslint:disable */
/* eslint-disable */
/**
*/
export enum Cell {
  Dead,
  Alive,
}
export class GameBoard {
  free(): void;
/**
*/
  tick(): void;
/**
* @param {number} x 
* @param {number} y 
* @param {Set<any>} start_state 
* @returns {GameBoard} 
*/
  static new(x: number, y: number, start_state: Set<any>): GameBoard;
/**
* @returns {number} 
*/
  width(): number;
/**
* @returns {number} 
*/
  height(): number;
/**
* @returns {number} 
*/
  cells(): number;
}
