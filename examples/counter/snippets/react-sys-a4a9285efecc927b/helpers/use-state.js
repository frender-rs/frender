export function use_state_object(initial_value) {
  const [state, set_state] = React.useState(initial_value);
  return { value: state, setter: { set_state } };
}
export function use_state_auto_clean(initial_value, clean) {
  const obj = use_state_object(initial_value);
  const state = obj.value;
  React.useEffect(() => () => void clean(state), [state]);
  return obj;
}
