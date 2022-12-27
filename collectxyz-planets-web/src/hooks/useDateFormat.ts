const useDateFormat = (timeStampNanoseconds: number) => {
  const dateTime = new Date(timeStampNanoseconds / 1000000)

  const dd = String(dateTime.getDate()).padStart(2, '0');
  const MM = String(dateTime.getMonth() + 1).padStart(2, '0'); //January is 0!
  const yyyy = dateTime.getFullYear();
  const HH = dateTime.getHours();
  const mm = dateTime.getMinutes();

  return dd + "/" + MM + "/" + yyyy + " (" + HH + ":" + mm + ")"
}
export default useDateFormat;
