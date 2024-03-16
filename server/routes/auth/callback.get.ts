import { withQuery, parsePath } from 'ufo'

export default defineEventHandler(async event => {
  //const userId = getQuery(event)['userId']
  const config = {
    clientId: "test",
    clientSecret: "QgBl9I2CD3eVhL7LFvkHrYUK7oKL3LE2",
    serverUrl: "http://localhost:1998",
    realm: "test",
    scope: ["openid"],
  };

  const query = getQuery(event)
  const { code } = query

  if (!code) {
    return createError({
        statusCode: 400,
        message: 'Missing authorization code',
      });
  }

  const realmURL = `${config.serverUrl}/realms/${config.realm}`
  const tokenURL = `${realmURL}/protocol/openid-connect/token`

  const tokens: any = await $fetch(tokenURL, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded',
    },
    body: new URLSearchParams({
      client_id: config.clientId,
      client_secret: config.clientSecret,
      grant_type: 'authorization_code',
      redirect_uri: "http://localhost:3000/auth/callback",
      code: code as string,
    }).toString(),
  }).catch((error) => {
    return { error }
  })


  setCookie(event, 'access_token', tokens.access_token, 
    {
        httpOnly: true,
        secure: true,
        sameSite: 'none',
        maxAge: tokens.expires_in,
    }
  )
  setCookie(event, 'refresh_token', tokens.refresh_token, {
        httpOnly: true,
        secure: true,
        sameSite: 'none',
        maxAge: tokens.refresh_expires_in,
    })

  return sendRedirect(event, "/")
})