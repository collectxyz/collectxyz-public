import { useHistory } from 'react-router'

const useNavigation = () => {
  const history = useHistory()
  const closeModal = () => history.replace({})
  return {closeModal}
}
export default useNavigation
