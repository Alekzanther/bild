import React from "react";
import "./Thumbnail.css";
interface ThumbnailProps {
  alt: string;
  thumb: string;
  src: string;
}
const Thumbnail = (props: ThumbnailProps) => {
  const [isLoaded, setIsLoaded] = React.useState(false);
  return (
    <React.Fragment>
      <img
        className="image thumb"
        alt={props.alt}
        src={props.thumb}
        style={{ visibility: isLoaded ? "hidden" : "visible" }}
      />
      <img
        onLoad={() => {
          setIsLoaded(true);
        }}
        className="image full"
        style={{ opacity: isLoaded ? 1 : 0 }}
        alt={props.alt}
        src={props.src}
      />
    </React.Fragment>
  );
};
export default Thumbnail;