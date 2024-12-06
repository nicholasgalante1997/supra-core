import React from 'react';
import Document from './components/Document';
import ComponentWithProps from './components/ComponentWithProps';

export default function AppWithProps({ message }) {
    return (
        <Document>
          <ComponentWithProps message={message} />
        </Document>
    );
}