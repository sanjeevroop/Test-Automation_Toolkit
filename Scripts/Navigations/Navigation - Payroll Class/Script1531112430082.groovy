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
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW

//WebUI.openBrowser('')
//WebUI.navigateToUrl('https://login.microsoftonline.com/7ef41b2d-b0df-48c8-8b0c-c6497fdf26ac/oauth2/authorize?client_id=f6722b0a-c5be-4f78-99b5-b3c01917fbfe&response_mode=form_post&response_type=code%20id_token&scope=openid%20profile&state=OpenIdConnect.AuthenticationProperties%3DbOseK9hKy3LrQrjoSp4bENAWIP9tGGBy4oPr2emf4iWSeUzZ75uMiS3iOuxycVv665_KKNklann6HGjlcR-5LV3w3GY9lEkSbPiiBdoeDutUaCUH3MiRCpMx5CbsUeHOC8lb7qjTGdUDvWlMx8C5vA&nonce=636667092534416338.OThiODNiYzAtZTAwMS00YmNkLTgyZDAtODM1YTA2ZTIzYzFjYjg1ZTdhZDMtMGI5NC00M2FjLThmNWMtZTZmOThkYjRmYWJi&x-client-SKU=ID_NET451&x-client-ver=5.2.1.0')
//WebUI.setText(findTestObject('Navigations/Navigation - Payroll Class/input_loginfmt'), 'Sanjeev@sdworx.com')
//WebUI.click(findTestObject('Navigations/Navigation - Payroll Class/input_idSIButton9'))
WebUI.delay(2)

WebUI.click(findTestObject('Navigations/Navigation - Payroll Class/span_Your Payroll Rules'))

WebUI.delay(2)

WebUI.click(findTestObject('Navigations/Navigation - Payroll Class/a_Payroll Class'))

WebUI.delay(2)

WebUI.click(findTestObject('Navigations/Navigation - Payroll Class/a_Add Class'))

WebUI.verifyTextPresent('Action', false)

