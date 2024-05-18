import React from "react";

interface Props {
  children?: React.ReactNode;
}

function Expandable({ children }: Props) {
  let [isExpanded, setIsExpanded] = React.useState(true);

  let handleClick = () => setIsExpanded(!isExpanded);

  if (isExpanded) {
    return (
      <>
        <button type="button" className="button" onClick={handleClick}>
          Minimize
        </button>
        {children}
      </>
    );
  } else {
    return (
      <button type="button" className="button" onClick={handleClick}>
        Expand
      </button>
    );
  }
}

export default Expandable;
