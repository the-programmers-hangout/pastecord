import { useEffect } from 'react';
import { useLoaderData } from 'react-router-dom';

type NewPasteContentProps = {
	content: string,
	onChange: (e: any) => any
}

export const NewPasteContent = (props: NewPasteContentProps) => {
	const paste = useLoaderData();
	return (
		<textarea className="mt-5 flex-1 px-5 bg-transparent font-mono h-100 w-screen border-0 resize-none border-0 outline-none"
      value={props.content} // ...force the input's value to match the state variable...
      onChange={(e) => props.onChange(e.target.value)} // ... and update the state variable on any edits!
			onDrop={() => false}
			autoComplete="false"
			spellCheck={false}
    />
	)
}