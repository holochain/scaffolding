import { AppWebsocket, HolochainError, type AppClient } from '@holochain/client';
import { createContext, FC, useEffect, useRef, useState } from 'react';

export const HolochainContext = createContext<HolochainContextValues>({
  client: undefined,
	error: undefined,
	loading: true,
});

const HolochainProvider: FC<HolochainProviderProps> = ({ children }) => {
  const [loading, setLoading] = useState(true);
	const [error, setError] = useState<HolochainError | undefined>();
  const client = useRef<AppClient>();
  
  const value = { client: client.current, error, loading };

  useEffect(() => {
    const connectClient = async () => {
      try {
        client.current = await AppWebsocket.connect();
      } catch (error) {
				setError(error as HolochainError)
        console.error('Failed to establish websocket connection:', error);
      } finally {
        setLoading(false);
      }
    };
    connectClient();
  }, []);

  return (
    <HolochainContext.Provider value={value}>
      {children}
    </HolochainContext.Provider>
  );
};

interface HolochainContextValues {
	client: AppClient | undefined,
	error: HolochainError | undefined,
	loading: boolean,
}

interface HolochainProviderProps {
  children: React.ReactNode;
}

export default HolochainProvider;
