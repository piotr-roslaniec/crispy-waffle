import { start } from '~prover';

// TODO: Expose this in the extension interface
function IndexPopup() {
  return <button onClick={() => start(8, 12)}>Start</button>;
}

export default IndexPopup;
