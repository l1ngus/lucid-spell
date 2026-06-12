import { useState, useRef, useEffect, createContext, type PropsWithChildren } from "react";
import { setMetaItem, getAllMetaItems, type UserMeta } from "../stores/userMetaStore";
import { UserMetaSchema } from '@/app/types/UserMeta';

type GetUserMetaFn = <K extends keyof UserMeta>(key: K) => UserMeta[K];
type SetUserMetaFn = <K extends keyof UserMeta>(key: K, value: UserMeta[K]) => void;

interface UserMetaContextValue {
  getUserMeta: GetUserMetaFn;
  setUserMeta: SetUserMetaFn;
}

export const UserMetaContext = createContext<UserMetaContextValue | null>(null);


export const UserMetaProvider = ({ children }: PropsWithChildren) => {
  const userMetaRef = useRef<UserMeta>(UserMetaSchema.parse({}));
  const [isLoaded, setIsLoaded] = useState(false);

  useEffect(() => {
    getAllMetaItems()
      .then(userMeta => {
        setIsLoaded(true);
        userMetaRef.current = userMeta;
      })
  }, [])

  const getUserMeta: GetUserMetaFn = (key) => {
    return userMetaRef.current[key];
  }

  const setUserMeta: SetUserMetaFn = (key, value) => {
    userMetaRef.current[key] = value;
    setMetaItem(key, value);
  }

  if (!isLoaded) return null;

  return (
    <UserMetaContext.Provider value={{ getUserMeta, setUserMeta }}>
      {children}
    </UserMetaContext.Provider>
  )
}


