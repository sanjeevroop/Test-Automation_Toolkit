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

//WebUI.navigateToUrl('https://login.microsoftonline.com/7ef41b2d-b0df-48c8-8b0c-c6497fdf26ac/oauth2/authorize?client_id=f6722b0a-c5be-4f78-99b5-b3c01917fbfe&response_mode=form_post&response_type=code%20id_token&scope=openid%20profile&state=OpenIdConnect.AuthenticationProperties%3DHsV0LpJa2zQBOfKap6up0e49qN0bsJGO-fxILLlOFBOmxS5wJio3qNX4KjxmeWBjw3pxfDEXlu4_lbiLcXlrjOXH6zp8G5kdQVwc31OqqGdUD_8MoJJn5rOVy9dA88gHqiS0qCmt1lachrbGeDtRsA&nonce=636686105244085666.YzMwZWExNDctN2MyMy00OGEwLTk0ZmEtOTBhM2YyNzI4YmM5ODQ4MDMyYjktMGJjYy00NzcyLTgyNDctNzNlZDhkYjI1NDFj&x-client-SKU=ID_NET451&x-client-ver=5.2.1.0')

//WebUI.setText(findTestObject('Page_Sign in to your account/input_loginfmt'), 'Sanjeev@sdworx.com')

//WebUI.click(findTestObject('Page_Sign in to your account/input_idSIButton9'))

//WebUI.click(findTestObject('Page_Implementation Toolkit/span_glyphicon glyphicon-menu-'))

//WebUI.click(findTestObject('Page_Implementation Toolkit/button_            Your Payrol'))

//WebUI.click(findTestObject('Page_Implementation Toolkit/a_Departments'))

WebUI.delay(2)

WebUI.setText(findTestObject('Delete Records/Delete - Department Details Cancel/Search'), findTestData('Search Department - Delete').getValue(1, 1))

WebUI.delay(2)

WebUI.click(findTestObject('Delete Records/Delete - Department Details Cancel/Delete'))

WebUI.delay(2)

WebUI.click(findTestObject('Delete Records/Delete - Department Details Cancel/button_No'))

