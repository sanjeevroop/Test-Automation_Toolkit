import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

//WebUI.openBrowser('')

//WebUI.navigateToUrl('https://login.microsoftonline.com/7ef41b2d-b0df-48c8-8b0c-c6497fdf26ac/oauth2/authorize?client_id=f6722b0a-c5be-4f78-99b5-b3c01917fbfe&response_mode=form_post&response_type=code%20id_token&scope=openid%20profile&state=OpenIdConnect.AuthenticationProperties%3D8WCAApwgHUwEeY8UbydynT9Tw0nlL9XRR2IpB1kAVxalE8gdMpFrLEySi01MokKBwFKsBpcjOcEP64Kfnp7Bzv5AJ4TIoVf6gjLRO8TePzRt0ZTSdllvs7Ps-jcblrtkbftRODgDeT2kJY1Wkj14yw&nonce=636700018101058956.NDc4YTRhNGItM2QzYi00NmEyLWExM2ItNTIyYTJjNGE4MDQ4MjlkMjkzZGUtN2JhOS00NjY1LWE4YjYtMWU2NzNkZDBkMzFk&x-client-SKU=ID_NET451&x-client-ver=5.2.1.0')

//WebUI.setText(findTestObject('Navigations/Navigation - Payroll Rates/input_loginfmt'), 
    //'Sanjeev@sdworx.com')

//WebUI.click(findTestObject('Navigations/Navigation - Payroll Rates/input_idSIButton9'))

WebUI.delay(5)

WebUI.click(findTestObject('Navigations/Navigation - Payroll Rates/span_Your Pay  Benefits'))

WebUI.delay(2)

WebUI.click(findTestObject('Navigations/Navigation - Payroll Rates/a_Rates'))

WebUI.click(findTestObject('Object Repository/Navigations/Navigation - Payroll Rates/Search Payroll Rates'))


