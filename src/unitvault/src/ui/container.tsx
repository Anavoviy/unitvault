import {children, JSXElement} from "solid-js";


export default function Container(props: {
    children: JSXElement,
    class: string
}) {

    var safe_children = children(() => props.children);

    return (
        <div class={"w-full h-full max-w-[1200px] mx-auto pt-10 pb-5 " + props.class}>
            {safe_children()}
        </div>
    )
}